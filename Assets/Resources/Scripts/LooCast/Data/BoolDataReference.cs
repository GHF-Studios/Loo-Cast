using System;
using UnityEngine.Events;

namespace LooCast.Data
{
    [Serializable]
    public class BoolDataReference
    {
        public bool UseConstant = false;
        public bool ConstantValue;
        public DynamicBoolData Variable;
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

        public static bool[] Evaluate(BoolDataReference[] boolDataReferences)
        {
            bool[] evaluatedValues = new bool[boolDataReferences.Length];
            for (int i = 0; i < boolDataReferences.Length; i++)
            {
                evaluatedValues[i] = boolDataReferences[i].Value;
            }
            return evaluatedValues;
        }
    } 
}
