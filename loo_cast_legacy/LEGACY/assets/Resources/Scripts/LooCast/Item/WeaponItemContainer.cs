using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Item
{
    using LooCast.Util;

    public class WeaponItemContainer : ItemContainer
    {
        public WeaponItemContainer(int slotCount, GameObject originObject = null) : base(slotCount, originObject)
        {
            
        }

        public WeaponItemContainer(WeaponItem[] weaponItems, GameObject originObject = null) : base(weaponItems, originObject)
        {
            
        }

        public override void TryAddItem(Item item, out Item remainingItem)
        {
            if (item is not WeaponItem)
            {
                throw new ArgumentException("Invalid Item Type!");
            }
            base.TryAddItem(item, out remainingItem);
        }

        public override void SetItem(int slotID, Item item)
        {
            if (item is not WeaponItem)
            {
                throw new ArgumentException("Invalid Item Type!");
            }
            base.SetItem(slotID, item);
        }

        public new WeaponItem GetItem(int slotID)
        {
            return (WeaponItem)base.GetItem(slotID);
        }

        public new WeaponItem[] GetItems()
        {
            return base.GetItems().Cast<WeaponItem>();
        }

        public override bool Contains(Item item)
        {
            if (item is not WeaponItem)
            {
                throw new ArgumentException("Invalid Item Type!");
            }
            return base.Contains(item);
        }
    }
}
