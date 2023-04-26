using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.MetaData
{
    using LooCast.System.Collections.Generic;
    using LooCast.System.Identifiers;

    [Serializable]
    public class SystemObjectMetaData : IInstanceMetaData
    {
        #region Properties
        public SystemObjectIdentifier SystemObjectIdentifier
        {
            get
            {
                return systemObjectIdentifier;
            }
            set
            {
                systemObjectIdentifier = value;
            }
        }
        public Guid SystemObjectInstanceGUID
        {
            get
            {
                return systemObjectIdentifier.SystemObjectInstanceGUID;
            }
        }
        public Type SystemObjectType
        {
            get
            {
                return TypeManager.Instance.GetType(systemObjectIdentifier.SystemObjectTypeIdentifier);
            }
        }
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
                parentSystemObjectIdentifier = parentSystemObject == null ? null : parentSystemObject.SystemObjectMetaData.SystemObjectIdentifier;
            }
        }
#nullable disable
        public List<SystemObject> ChildSystemObjects
        {
            get
            {
                if (childSystemObjects == null)
                {
                    childSystemObjects = new List<SystemObject>();
                    if (childSystemObjectIdentifiers.Count > 0)
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
                childSystemObjectIdentifiers = new SerializableList<SystemObjectIdentifier>();
                if (childSystemObjects != null && childSystemObjects.Count > 0)
                {
                    for (int i = 0; i < childSystemObjects.Count; i++)
                    {
                        childSystemObjectIdentifiers.Add(childSystemObjects[i].SystemObjectMetaData.SystemObjectIdentifier);
                    }
                }
            }
        }
        #endregion

        #region Fields
        [SerializeField] private SystemObjectIdentifier systemObjectIdentifier;
#nullable enable
        [SerializeField] private SystemObjectIdentifier? parentSystemObjectIdentifier;
#nullable disable
        [SerializeField] private SerializableList<SystemObjectIdentifier> childSystemObjectIdentifiers;

#nullable enable
        private SystemObject? parentSystemObject;
        private List<SystemObject>? childSystemObjects;
#nullable disable
        #endregion

        #region Methods
        public virtual bool Validate()
        {
            if (systemObjectIdentifier == null)
            {
                return false;
            }
            if (childSystemObjectIdentifiers == null)
            {
                return false;
            }
            return true;
        }
        #endregion
    }
}
