using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Item
{
    using Attribute.Stat;
    using Variable;

    public interface IUpgradableItem
    {
        void ApplyItemStatUpgradeSet(int upgradeSetID, Stats stats);
        void RemoveItemStatUpgradeSet(int upgradeSetID);
    }
}