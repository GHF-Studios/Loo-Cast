using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Inventory
{
    using Data;
    using Item;
    using Item.Data;
    using LooCast.Core;
    using LooCast.System;

    public sealed class AllyStationInventory : ExtendedMonoBehaviour
    {
        #region Data
        public AllyStationInventoryData Data;
        #endregion

        #region Properties
        public WeaponItemContainer WeaponItemContainer { get; private set; }
        #endregion

        #region Fields
        [SerializeField] private MultiplexerWeaponItemData stationMultiplexerWeaponItemData;
        [SerializeField] private LaserEmitterWeaponItemData stationLaserEmitterWeaponItemData;
        #endregion

        #region Methods
        private void Start()
        {
            WeaponItemContainer = new WeaponItemContainer(Data.SlotCount.Value, gameObject);

            MultiplexerWeaponItem multiplexerWeaponItem = (MultiplexerWeaponItem)stationMultiplexerWeaponItemData.CreateItem();
            WeaponItemContainer.SetItem(0, multiplexerWeaponItem);

            //LaserEmitterWeaponItem laserEmitterWeaponItem = (LaserEmitterWeaponItem)stationLaserEmitterWeaponItemData.CreateItem();
            //WeaponItemContainer.SetItem(1, laserEmitterWeaponItem);
        }
        #endregion
    }
}