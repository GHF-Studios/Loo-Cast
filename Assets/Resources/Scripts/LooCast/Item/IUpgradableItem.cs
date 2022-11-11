using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Item
{
    using Attribute.Stat;
    using Variable;

    public interface IUpgradableItem
    {
        Dictionary<Stat.Stat, Increase> activeStatIncreases { get; }
        Dictionary<Stat.Stat, Multiplier> activeStatMultipliers { get; }
        void ApplyItemUpgrade(Stat.Stat stat);
        void RemoveItemUpgrade(Stat.Stat stat);
    }
}