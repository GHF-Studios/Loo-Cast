using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Inventory
{
    using Data;
    using Data.Runtime;
    using LooCast.Item;
    using LooCast.Targeting;
    using LooCast.Player;
    using LooCast.Attribute.Stat;

    public sealed class PlayerInventory : MonoBehaviour
    {
        #region Data
        public PlayerInventoryData Data;
        public PlayerInventoryRuntimeData RuntimeData;
        #endregion

        #region Properties

        #endregion

        #region Fields
        [SerializeField] private Player player;
        [SerializeField] private ITargeting targeting;
        [SerializeField] private Stats stats;
        #endregion

        #region Unity Callbacks
        private void Start()
        {
            RuntimeData.Initialize(Data);

            Player player = GetComponentInParent<Player>();

            //ChargedPlasmaLauncherWeaponItem chargedPlasmaLauncherWeaponItem = new ChargedPlasmaLauncherWeaponItem(Data.ChargedPlasmaLauncherWeaponItemData, null, stats);
            //chargedPlasmaLauncherWeaponItem.OnPickup.Invoke(gameObject);
            //RuntimeData.Hotbar.AddItem(chargedPlasmaLauncherWeaponItem, out Item remainingChargedPlasmaLauncherWeaponItem);
            //
            //FreezeRayWeaponItem freezeRayWeaponItem = new FreezeRayWeaponItem(Data.FreezeRayWeaponItemData, null, stats);
            //freezeRayWeaponItem.OnPickup.Invoke(gameObject);
            //RuntimeData.Hotbar.AddItem(freezeRayWeaponItem, out Item remainingFreezeRayWeaponItem);
            
            LaserEmitterWeaponItem laserEmitterWeaponItem = new LaserEmitterWeaponItem(Data.LaserEmitterWeaponItemData, null, stats);
            laserEmitterWeaponItem.OnPickup.Invoke(gameObject);
            RuntimeData.Hotbar.AddItem(laserEmitterWeaponItem, out Item remainingLaserEmitterWeaponItem);
            
            MultiplexerWeaponItem multiplexerWeaponItem = new MultiplexerWeaponItem(Data.MultiplexerWeaponItemData, null, stats);
            multiplexerWeaponItem.OnPickup.Invoke(gameObject);
            RuntimeData.Hotbar.AddItem(multiplexerWeaponItem, out Item remainingMultiplexerWeaponItem);
        }
        #endregion

        #region Methods

        #endregion
    }
}