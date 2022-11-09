using System;
using UnityEngine.Events;

namespace LooCast.Item
{
    using Data;

    public abstract class AmountableItem : Item
    {
        #region Data
        public AmountableItemData AmountableItemData { get; private set; }
        #endregion

        public UnityEvent OnAmountChanged
        {
            get
            {
                return onAmountChanged;
            }
        }
        public float MaxAmount { get; protected set; }
        public float Amount
        {
            get
            {
                return amount;
            }

            set
            {
                if (value > MaxAmount)
                {
                    throw new ArgumentOutOfRangeException("Amount cannot be greater than MaxAmount!");
                }
                else if (value < 0)
                {
                    throw new ArgumentOutOfRangeException("Amount cannot be less than or equal to 0!");
                }
                else
                {
                    amount = value;
                    onAmountChanged.Invoke();
                }
            }
        }
        public float Density { get; protected set; }
        public float MinObjectScale { get; protected set; }
        public float MaxObjectScale { get; protected set; }

        private float amount;
        private UnityEvent onAmountChanged;

        public AmountableItem(AmountableItemData data) : base(data)
        {
            AmountableItemData = data;

            onAmountChanged = new UnityEvent();

            MaxAmount = data.MaxAmount.Value;
            Amount = data.DefaultAmount.Value;
            Density = data.Density.Value;
            MinObjectScale = data.MinObjectScale.Value;
            MaxObjectScale = data.MaxObjectScale.Value;
        }

        public bool IsFull()
        {
            return Amount >= MaxAmount;
        }

        public bool CanFit(float amount)
        {
            return Amount + amount <= MaxAmount;
        }

        public float GetFreeAmount()
        {
            return MaxAmount - Amount;
        }
    }
}