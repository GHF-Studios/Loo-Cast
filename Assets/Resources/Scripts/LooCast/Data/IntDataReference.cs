using System;
using UnityEngine.Events;

namespace LooCast.Data
{
    [Serializable]
    public class IntDataReference
    {
        public bool UseConstant = true;
        public int ConstantValue;
        public IntData Variable;
        public UnityEvent OnValueChanged = new UnityEvent();

        public int Value
        {
            get
            {
                return UseConstant ? ConstantValue : Variable.Value;
            }

            set
            {
                if (UseConstant)
                {
                    ConstantValue = value;
                }
                else
                {
                    Variable.Value = value;
                }
                OnValueChanged.Invoke();
            }
        }
    } 
}
