using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Item
{
    using Attribute.Stat;
    using Variable;

    public interface IUpgradableItem
    {
        void ApplyItemStatUpgradeSet(int upgradeSetID, UpgradeSet upgradeSet);
        void RemoveItemStatUpgradeSet(int upgradeSetID);
    }
}