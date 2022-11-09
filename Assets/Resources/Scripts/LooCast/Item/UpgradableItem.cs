using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Item
{
    using Attribute.Stat;
    using LooCast.Item.Data;
    using Variable;

    public abstract class UpgradableItem : UniqueItem
    {
        #region Data
        public UpgradableItemData UpgradableItemData { get; private set; }
        #endregion

        public UpgradableItem(UpgradableItemData data) : base(data)
        {
            UpgradableItemData = data;
        }

        public abstract void ApplyItemStatUpgradeSet(int upgradeSetID, UpgradeSet upgradeSet);
        public abstract void RemoveItemStatUpgradeSet(int upgradeSetID);
    }
}