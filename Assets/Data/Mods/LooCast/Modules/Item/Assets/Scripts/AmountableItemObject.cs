using System;
using UnityEngine;

namespace LooCast.Item
{
    using Data;

    public abstract class AmountableItemObject : ItemObject
    {
        public AmountableItem AmountableItem { get; protected set; }
        public override Item Item
        {
            set
            {
                AmountableItem = (AmountableItem)value;
                if (AmountableItem == null)
                {
                    throw new ArgumentException("Invalid Item Type!");
                }
                base.Item = value;
            }
        }

        protected void Initialize(AmountableItem amountableItem)
        {
            base.Initialize(amountableItem);
            AmountableItem = amountableItem;
        }
    }
}