using System;
using UnityEngine;

namespace LooCast.Item
{
    using Data;

    public abstract class AmountableItemObject : ItemObject
    {
        public AmountableItem AmountableItem { get; protected set; }

        protected void Initialize(AmountableItem amountableItem)
        {
            base.Initialize(amountableItem);
            AmountableItem = amountableItem;
        }
    }
}