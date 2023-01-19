using System;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Data
{
    [CreateAssetMenu(fileName = "DynamicFloatData", menuName = "Data/DynamicFloatData", order = 0)]
    public class DynamicFloatData : DynamicData
    {
        #region Properties
        public float Value
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
        public override Type DataType
        {
            get
            {
                return typeof(DynamicFloatData);
            }
        }
        public override DataFolder ParentFolder
        {
            get
            {
                return (DataFolder)DataManager.Instance.GetDataFolder(parentFolderID);
            }
        }
        #endregion

        #region Fields
        public UnityEvent OnValueChanged;
        
        [SerializeField] private float value;
        [SerializeField] private string parentFolderID;
        #endregion
    } 
}
