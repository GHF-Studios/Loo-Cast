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
        public ItemContainer<WeaponItem> WeaponItemContainer { get; private set; }
        #endregion

        #region Fields
        [SerializeField] private PlayerStation playerStation;
        [SerializeField] private ITargeting targeting;
        [SerializeField] private Stats stats;
        [SerializeField] private MultiplexerWeaponItemData stationMultiplexerWeaponItemData;
        #endregion

        private void Start()
        {
            WeaponItemContainer = new ItemContainer<WeaponItem>(Data.SlotCount.Value);

            MultiplexerWeaponItem multiplexerWeaponItem = new MultiplexerWeaponItem(stationMultiplexerWeaponItemData, null, stats, true);
            multiplexerWeaponItem.OnPickup.Invoke(gameObject);
            WeaponItemContainer.SetItem(0, multiplexerWeaponItem);
        }
    }
}