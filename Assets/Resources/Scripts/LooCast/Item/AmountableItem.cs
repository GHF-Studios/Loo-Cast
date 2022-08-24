using System;

namespace LooCast.Item
{
    using Data;

    public abstract class AmountableItem : Item
    {
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
                }
            }
        }
        private float amount;

        public AmountableItem(AmountableItemData data) : base(data)
        {
            MaxAmount = data.MaxAmount;
            Amount = data.DefaultAmount;
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