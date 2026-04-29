using System;
using UnityEngine;

namespace LooCast.Item
{
    using Data;

    public abstract class WeaponItemObject : UpgradableItemObject
    {
        public WeaponItem WeaponItem { get; protected set; }
        public override Item Item
        {
            set
            {
                WeaponItem = (WeaponItem)value;
                if (WeaponItem == null)
                {
                    throw new ArgumentException("Invalid Item Type!");
                }
                base.Item = value;
            }
        }

        protected void Initialize(WeaponItem weaponItem)
        {
            base.Initialize(weaponItem);
            WeaponItem = weaponItem;
        }
    }
}