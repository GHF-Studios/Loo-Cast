using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Inventory
{
    using Data;
    using LooCast.Item;

    public sealed class PlayerStationInventory : Inventory
    {
        #region Data
        public PlayerStationInventoryData Data { get; private set; }
        #endregion

        public ItemContainer WeaponItemContainer { get; private set; }

        public void Initialize(PlayerStationInventoryData data)
        {
            WeaponItemContainer = new ItemContainer(data.DefaultWeapons, (item) => { return item is WeaponItem; } );
        }
    }
}