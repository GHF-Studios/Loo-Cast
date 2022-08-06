using System;
using UnityEngine.Events;

namespace LooCast.Data
{
    [Serializable]
    public class BoolDataReference
    {
        public bool UseConstant = true;
        public bool ConstantValue;
        public BoolData Variable;
        public UnityEvent OnValueChanged = new UnityEvent();

        public bool Value
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
