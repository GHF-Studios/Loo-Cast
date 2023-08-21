using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Inventory
{
    using Data;
    using Data.Runtime;
    using LooCast.Item;
    using LooCast.Player;
    using LooCast.Attribute.Stat;
    using LooCast.Core;

    public sealed class PlayerInventory : Component
    {
        #region Data
        public PlayerInventoryData Data;
        public PlayerInventoryRuntimeData RuntimeData;
        #endregion

        #region Properties

        #endregion

        #region Fields
        [SerializeField] private Player player;
        [SerializeField] private Stats stats;
        #endregion

        #region Unity Callbacks
        private void Start()
        {
            Player player = GetComponentInParent<Player>();

            RuntimeData.Initialize(Data, player);

            //ChargedPlasmaLauncherWeaponItem chargedPlasmaLauncherWeaponItem = (ChargedPlasmaLauncherWeaponItem)Data.ChargedPlasmaLauncherWeaponItemData.CreateItem();
            //RuntimeData.Hotbar.AddItem(chargedPlasmaLauncherWeaponItem, out Item remainingChargedPlasmaLauncherWeaponItem);
            
            //FreezeRayWeaponItem freezeRayWeaponItem = (FreezeRayWeaponItem)Data.FreezeRayWeaponItemData.CreateItem();
            //RuntimeData.Hotbar.AddItem(freezeRayWeaponItem, out Item remainingFreezeRayWeaponItem);
            
            LaserEmitterWeaponItem laserEmitterWeaponItem = (LaserEmitterWeaponItem)Data.LaserEmitterWeaponItemData.CreateItem();
            RuntimeData.Hotbar.TryAddItem(laserEmitterWeaponItem, out Item remainingLaserEmitterWeaponItem);
            
            //MultiplexerWeaponItem multiplexerWeaponItem = (MultiplexerWeaponItem)Data.MultiplexerWeaponItemData.CreateItem();
            //RuntimeData.Hotbar.TryAddItem(multiplexerWeaponItem, out Item remainingMultiplexerWeaponItem);
        }
        #endregion

        #region Methods

        #endregion
    }
}