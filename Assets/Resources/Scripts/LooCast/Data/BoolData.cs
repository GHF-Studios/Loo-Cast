using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Data
{
    [CreateAssetMenu(fileName = "BoolData", menuName = "Data/BoolData", order = 0)]
    public class BoolData : ScriptableObject
    {
        [SerializeField] private bool value;
        public bool Value
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
