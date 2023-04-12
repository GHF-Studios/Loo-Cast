using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.MetaData
{
    using global::LooCast.System.Identifiers;
    using global::LooCast.System.Managers;
    
    [Serializable]
    public class SystemObjectMetaData<T> where T : SystemObject
    {
        #region Properties
#nullable enable
        public SystemObject? ParentSystemObject
        {
            get
            {
                if (parentSystemObject == null)
                {
                    if (parentSystemObjectIdentifier != null)
                    {
                        parentSystemObject = SystemObjectManager.Instance.GetSystemObject(parentSystemObjectIdentifier);
                    }
                }
                return parentSystemObject;
            }
            set
            {
                parentSystemObject = value;
            }
        }
#nullable disable
        public HashSet<SystemObject> ChildSystemObjects
        {
            get
            {
                if (childSystemObjects == null)
                {
                    childSystemObjects = new HashSet<SystemObject>();
                    if (childSystemObjectIdentifiers.Length > 0)
                    {
                        foreach (SystemObjectIdentifier childSystemObjectIdentifier in childSystemObjectIdentifiers)
                        {
                            SystemObject childSystemObject = SystemObjectManager.Instance.GetSystemObject(childSystemObjectIdentifier);
                            childSystemObjects.Add(childSystemObject);
                        }
                    }
                }
                return childSystemObjects;
            }
            set
            {
                childSystemObjects = value;
            }
        }
        #endregion

        #region Fields
#nullable enable
        [SerializeField] private SystemObjectIdentifier? parentSystemObjectIdentifier;
#nullable disable
        [SerializeField] private SystemObjectIdentifier[] childSystemObjectIdentifiers;

#nullable enable
        private SystemObject? parentSystemObject;
        private HashSet<SystemObject>? childSystemObjects;
#nullable disable
        #endregion
    }
}
