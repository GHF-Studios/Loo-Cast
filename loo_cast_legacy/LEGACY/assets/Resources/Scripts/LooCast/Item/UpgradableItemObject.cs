using System;
using UnityEngine;

namespace LooCast.Item
{
    using Data;

    public abstract class UpgradableItemObject : UniqueItemObject
    {
        public UpgradableItem UpgradableItem { get; protected set; }
        public override Item Item
        {
            set
            {
                UpgradableItem = (UpgradableItem)value;
                if (UpgradableItem == null)
                {
                    throw new ArgumentException("Invalid Item Type!");
                }
                base.Item = value;
            }
        }

        protected void Initialize(UpgradableItem upgradableItem)
        {
            base.Initialize(upgradableItem);
            UpgradableItem = upgradableItem;
        }
    }
}