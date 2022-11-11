using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Item
{
    public interface IItemUpgrader
    {
        UpgradeSet UpgradeSet { get; }
    }
}