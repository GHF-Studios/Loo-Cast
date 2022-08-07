using System;
using UnityEngine.Events;

namespace LooCast.Data
{
    [Serializable]
    public class FloatDataReference
    {
        public bool UseConstant = false;
        public float ConstantValue;
        public FloatData Variable;
        public UnityEvent OnValueChanged = new UnityEvent();

        public float Value
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
