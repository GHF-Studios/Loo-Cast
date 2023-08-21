using System;
using UnityEngine.Events;

namespace LooCast.Data
{
    [Serializable]
    public class IntDataReference
    {
        public bool UseConstant = false;
        public int ConstantValue;
        public DynamicIntData Variable;
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

        public static int[] Evaluate(IntDataReference[] intDataReferences)
        {
            int[] evaluatedValues = new int[intDataReferences.Length];
            for (int i = 0; i < intDataReferences.Length; i++)
            {
                evaluatedValues[i] = intDataReferences[i].Value;
            }
            return evaluatedValues;
        }
    } 
}
