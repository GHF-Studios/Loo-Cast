using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Inventory
{
    using Data;
    using LooCast.Item;
    using LooCast.Item.Data;
    using LooCast.Station;
    using LooCast.Attribute.Stat;

    public sealed class PlayerStationInventory : MonoBehaviour
    {
        #region Data
        public PlayerStationInventoryData Data;
        #endregion

        #region Properties
        public WeaponItemContainer WeaponItemContainer { get; private set; }
        #endregion

        #region Fields
        [SerializeField] private AllyStation allyStation;
        [SerializeField] private Stats stats;
        [SerializeField] private MultiplexerWeaponItemData stationMultiplexerWeaponItemData;
        #endregion

        private void Start()
        {
            WeaponItemContainer = new WeaponItemContainer(Data.SlotCount.Value, allyStation.gameObject);

            MultiplexerWeaponItem multiplexerWeaponItem = (MultiplexerWeaponItem)stationMultiplexerWeaponItemData.CreateItem();
            WeaponItemContainer.SetItem(0, multiplexerWeaponItem);
        }
    }
}