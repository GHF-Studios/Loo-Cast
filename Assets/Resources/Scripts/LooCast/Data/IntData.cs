using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Data
{
    [CreateAssetMenu(fileName = "IntData", menuName = "Data/IntData", order = 0)]
    public class IntData : ScriptableObject
    {
        [SerializeField] private int value;
        public int Value
        {
            get
            {
                return value;
            }

            set
            {
                this.value = value;
                OnValueChanged.Invoke();
            }
        }
        public UnityEvent OnValueChanged;
    } 
}
