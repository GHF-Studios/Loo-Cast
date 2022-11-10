using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Inventory
{
    using Data;
    using Item;
    using Item.Data;
    using Core;

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
        #endregion

        #region Methods
        private void Start()
        {
            WeaponItemContainer = new WeaponItemContainer(Data.SlotCount.Value, gameObject);

            MultiplexerWeaponItem multiplexerWeaponItem = (MultiplexerWeaponItem)stationMultiplexerWeaponItemData.CreateItem();
            WeaponItemContainer.SetItem(0, multiplexerWeaponItem);
        }
        #endregion
    }
}