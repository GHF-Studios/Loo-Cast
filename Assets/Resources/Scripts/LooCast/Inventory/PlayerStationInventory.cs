using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Inventory
{
    using Data;
    using LooCast.Item;

    public sealed class PlayerStationInventory : MonoBehaviour
    {
        #region Data
        public PlayerStationInventoryData Data { get; private set; }
        #endregion

        public ItemContainer WeaponItemContainer { get; private set; }

        public void Initialize(PlayerStationInventoryData data)
        {
            WeaponItemContainer = new ItemContainer(data.DefaultWeapons.Length, (item) => { return item is WeaponItem; } );
        }
    }
}