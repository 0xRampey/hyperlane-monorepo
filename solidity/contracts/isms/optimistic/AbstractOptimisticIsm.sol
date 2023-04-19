// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity >=0.8.0;

// ============ External Imports ============
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {console} from "forge-std/console.sol";

// ============ Internal Imports ============
import {IOptimisticIsm} from "../../interfaces/isms/IOptimisticIsm.sol";
import {MetaProxy} from "../../libs/MetaProxy.sol";
import {IInterchainSecurityModule} from "../../interfaces/IInterchainSecurityModule.sol";
import {Message} from "../../libs/Message.sol";
import {IMultisigIsm} from "../../interfaces/isms/IMultisigIsm.sol";
import {OptimisticIsmMetadata} from "../../libs/isms/OptimisticIsmMetadata.sol";

/**
 * @title OptimisticIsm
 * @notice Manages n per-domain ISM sets, any 1 of which is required
 * to verify interchain messages
 */
abstract contract AbstractOptimisticIsm is IOptimisticIsm {
    // ============ Constants ============

    // solhint-disable-next-line const-name-snakecase
    uint8 public constant moduleType =
        uint8(IInterchainSecurityModule.Types.OPTIMISTIC);

    // ============ Virtual Functions ============
    // ======= OVERRIDE THESE TO IMPLEMENT =======

    /**
     * @notice Returns the ISM that is responsible for verifying _message
     * @dev Can change based on the content of _message
     * @param _message Hyperlane formatted interchain message
     * @return modules The ISM address
     */
    function getPreVerifyIsm(bytes calldata _message)
        public
        view
        virtual
        override
        returns (address);

    /**
     * @notice Returns the set of watchers responsible for checking fraud _message
     * and the number of signatures required
     * @dev Can change based on the content of _message
     * @return watchers The array of validator addresses
     * @return threshold The number of validator signatures needed
     */
    function watchersAndThresholdAndFraudWindow(bytes memory)
        public
        view
        virtual
        override
        returns (
            address[] memory,
            uint8,
            uint32
        );

    // ============ Public Functions ============

    /**
     * @notice Requires that the chosen ISM has verified '_message'
     * @param _metadata ABI encoded module metadata (see OptimisticIsmMetadata.sol)
     * @param _message Formatted Hyperlane message (see Message.sol).
     */
    function preVerify(bytes calldata _metadata, bytes calldata _message)
        public
        returns (bool)
    {
        address ism = getPreVerifyIsm(_message);
        IInterchainSecurityModule _ism = IInterchainSecurityModule(ism);
        require(
            _ism.verify(OptimisticIsmMetadata.metadataAt(_metadata), _message),
            "!verify"
        );
        _setupFraudWindow(_metadata, _message);
        return true;
    }

    modifier onlyWatcher() {
        require(_isWatcher(msg.sender), "Caller is not a watcher");
        _;
    }

    function _isWatcher(address _watcher) private view returns (bool) {
        (address[] memory _watchers, , ) = watchersAndThresholdAndFraudWindow(
            abi.encodePacked("")
        );

        for (uint256 i = 0; i < _watchers.length; i++) {
            if (_watchers[i] == _watcher) {
                return true;
            }
        }
        return false;
    }

    mapping(address => bool) private _watcherMarkedFraudulent;
    uint256 _fraudulentCount = 0;

    /**
     * @notice Allows watchers to flag the ISM subModule as fraudulent
     * @param _ism Address of ISM subModule
     *
     */
    function markFraudulent(address _ism) external onlyWatcher returns (bool) {
        // Add check here for the _ism address
        require(
            !_watcherMarkedFraudulent[msg.sender],
            "Watcher has already marked fraudulent"
        );
        _watcherMarkedFraudulent[msg.sender] = true;
        _fraudulentCount += 1;
        return true;
    }

    /**
     * @notice Requires that m-of-n watchers sign '_message'
     * and agree on fraudulence of '_message'
     * @param _metadata ABI encoded module metadata (see OptimisticIsmMetadata.sol)
     * @param _message Formatted Hyperlane message (see Message.sol).
     */
    function verify(bytes calldata _metadata, bytes calldata _message)
        public
        returns (bool)
    {
        require(preVerify(_metadata, _message), "!verify");
        require(_verifySubIsmNotFraudulent(_message), "!fraud");
        require(_verifyTargetBlockReached(_metadata, _message), "!wait");
        return true;
    }

    // ============ Private Functions ============

    mapping(bytes32 => uint256) private _targetBlocks;

    /**
     * @notice Sets up a fraud window for a given message
     * @return bool If setup was succesful
     */
    function _setupFraudWindow(
        bytes calldata _metadata,
        bytes calldata _message
    ) internal returns (bool) {
        bytes32 messageHash = keccak256(abi.encodePacked(_metadata, _message));
        uint256 targetBlock = _targetBlocks[messageHash];
        if (targetBlock == 0) {
            (, , uint32 _fraudWindow) = watchersAndThresholdAndFraudWindow(
                _message
            );
            //First time we're dealing with this message,metadta pair
            _targetBlocks[messageHash] = block.number + _fraudWindow;
            return true;
        }
        return false;
    }

    /**
     * @notice Verifies that a quorum of watchers signed
     * the given message.
     * @param _metadata ABI encoded module metadata (see MultisigIsmMetadata.sol)
     * @param _message Formatted Hyperlane message (see Message.sol).
     */
    function _verifyTargetBlockReached(
        bytes calldata _metadata,
        bytes calldata _message
    ) internal returns (bool) {
        bytes32 messageHash = keccak256(abi.encodePacked(_metadata, _message));
        uint256 targetBlock = _targetBlocks[messageHash];

        if (block.number >= targetBlock) {
            // Clear out memory
            _targetBlocks[messageHash] = 0;
            return true;
        }
        return false;
    }

    /**
     * @notice Verifies that a quorum of watchers signed
     * the given message.
     */
    function _verifySubIsmNotFraudulent(bytes calldata _message)
        internal
        view
        returns (bool)
    {
        (, uint8 _threshold, ) = watchersAndThresholdAndFraudWindow(_message);
        return !(_fraudulentCount >= _threshold);
    }
}
