using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Inventory
{
    using Data;
    using LooCast.Item;
    using LooCast.Item.Data;
    using LooCast.Targeting;
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
        [SerializeField] private PlayerStation playerStation;
        [SerializeField] private ITargeting targeting;
        [SerializeField] private Stats stats;
        [SerializeField] private MultiplexerWeaponItemData stationMultiplexerWeaponItemData;
        #endregion

        private void Start()
        {
            WeaponItemContainer = new WeaponItemContainer(Data.SlotCount.Value, playerStation.gameObject);

            MultiplexerWeaponItem multiplexerWeaponItem = new MultiplexerWeaponItem(stationMultiplexerWeaponItemData, null, stats, true);
            WeaponItemContainer.SetItem(0, multiplexerWeaponItem);
        }
    }
}