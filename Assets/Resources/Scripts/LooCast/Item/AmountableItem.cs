using System;
using UnityEngine.Events;

namespace LooCast.Item
{
    using Data;

    public abstract class AmountableItem : Item
    {
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

        private float amount;
        private UnityEvent onAmountChanged;

        public AmountableItem(AmountableItemData data) : base(data)
        {
            onAmountChanged = new UnityEvent();

            MaxAmount = data.MaxAmount;
            Amount = data.DefaultAmount;
            Density = data.Density;
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