using System;
using UnityEngine;

namespace LooCast.Item
{
    using Data;

    public abstract class AmountableItemObject : ItemObject
    {
        protected new void Initialize(Item item)
        {
            base.Initialize(item);
        }
    }
}