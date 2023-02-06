using System;
using UnityEngine;

namespace LooCast.Data
{
    public abstract class DynamicData : ScriptableObject, IData
    {
        #region Properties
        public string ID
        {
            get
            {
                if (ParentFolder == null)
                {
                    return Name;
                }
                else
                {
                    return ParentFolder.ID + "." + Name;
                }
            }
        }
        public string Name
        {
            get
            {
                return name;
            }
        }

        public abstract Type DataType { get; }
        public abstract DataFolder ParentFolder { get; }
        #endregion
    }
}
