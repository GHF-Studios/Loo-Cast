using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Identifier
{
    using Util.Collections.Generic;
    
    [Serializable]
    public class InstanceIdentifier : IIdentifiableInstance
    {
        #region Properties
        public string ID
        {
            get
            {
                if (ParentInstance == null)
                {
                    return $"{InstanceType.ID}.{InstanceID}";
                }
                else
                {
                    return $"{ParentInstance.ID}.{InstanceType.ID}.{InstanceID}";
                }
            }
        }

        public IIdentifiableInstance ParentInstance => parentInstance;
        public List<IIdentifiableInstance> ChildInstances => childInstances.Values;
        public IIdentifiableType InstanceType => instanceType;
        public Guid InstanceID => instanceID;
        #endregion

        #region Fields
        [SerializeField] private IIdentifiableInstance parentInstance;
        [SerializeField] private SerializableList<IIdentifiableInstance> childInstances;
        [SerializeField] private IIdentifiableType instanceType;
        [SerializeField] private Guid instanceID;
        #endregion

        #region Constructors
        internal InstanceIdentifier(IIdentifiableType instanceType, Guid instanceID)
        {
            this.instanceType = instanceType;
            this.instanceID = instanceID;
            parentInstance = null;
            childInstances = null;
        }

        internal InstanceIdentifier(IIdentifiableType instanceType, Guid instanceID, IIdentifiableInstance parentInstance)
        {
            this.instanceType = instanceType;
            this.instanceID = instanceID;
            this.parentInstance = parentInstance;
            childInstances = null;
        }
        #endregion

        #region Methods
        public void AddChildInstance(IIdentifiableInstance childInstance)
        {
            if (childInstances == null)
            {
                childInstances = new SerializableList<IIdentifiableInstance>();
            }
            if (childInstances.Contains(childInstance))
            {
                throw new ArgumentException($"Child instance '{childInstance.ID}' already exists in '{ID}'!");
            }
            childInstances.Add(childInstance);
        }

        public void AddChildInstances(IEnumerable<IIdentifiableInstance> childInstances)
        {
            if (this.childInstances == null)
            {
                this.childInstances = new SerializableList<IIdentifiableInstance>();
            }
            foreach (IIdentifiableInstance childInstance in childInstances)
            {
                if (this.childInstances.Contains(childInstance))
                {
                    throw new ArgumentException($"Child instance '{childInstance.ID}' already exists in '{ID}'!");
                }
                this.childInstances.Add(childInstance);
            }
        }

        public void RemoveChildInstance(IIdentifiableInstance childInstance)
        {
            if (childInstances == null)
            {
                throw new NullReferenceException();
            }
            if (!childInstances.Contains(childInstance))
            {
                throw new ArgumentException($"Child instance '{childInstance.ID}' does not exist in '{ID}'!");
            }
            childInstances.Remove(childInstance);
        }
        #endregion
    }
}