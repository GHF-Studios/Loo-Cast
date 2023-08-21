using System;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Data
{
    [CreateAssetMenu(fileName = "DynamicStringData", menuName = "Data/DynamicStringData", order = 0)]
    public class DynamicStringData : DynamicData
    {
        #region Properties
        public string Value
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
                return typeof(DynamicStringData);
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
        
        [SerializeField] private string value;
        [SerializeField] private string parentFolderID;
        #endregion
    }
}
