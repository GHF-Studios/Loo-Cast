using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Inventory
{
    using Data;
    using Item;
    using Item.Data;
    using Attribute.Stat;
    using Core;

    public sealed class EnemyStationInventory : ExtendedMonoBehaviour
    {
        #region Data
        public EnemyStationInventoryData Data;
        #endregion

        #region Properties
        public WeaponItemContainer WeaponItemContainer { get; private set; }
        #endregion

        #region Fields
        [SerializeField] private Stats stats;
        [SerializeField] private MultiplexerWeaponItemData stationMultiplexerWeaponItemData;
        #endregion

        private void Start()
        {
            WeaponItemContainer = new WeaponItemContainer(Data.SlotCount.Value, gameObject);

            MultiplexerWeaponItem multiplexerWeaponItem = (MultiplexerWeaponItem)stationMultiplexerWeaponItemData.CreateItem();
            WeaponItemContainer.SetItem(0, multiplexerWeaponItem);
        }
    }
}