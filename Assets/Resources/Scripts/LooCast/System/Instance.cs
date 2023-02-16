using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using Identification;
    
    public class Instance : IInstance
    {
        #region Properties
        public object RootObject => rootObject;
        public IInstanceIdentifier InstanceIdentifier => instanceIdentifier;
        public IIdentifier Identifier => instanceIdentifier;
        public Instance ParentInstance => parentInstance;
        public List<Instance> ChildInstances => childInstances;
        #endregion

        #region Fields
        private object rootObject;
        private InstanceIdentifier instanceIdentifier;
        private Instance parentInstance;
        private List<Instance> childInstances;
        #endregion

        #region Constructors
        internal Instance(object rootObject, Type instanceType)
        {
            instanceIdentifier = new InstanceIdentifier((TypeIdentifier)instanceType.Identifier);
            parentInstance = null;
            childInstances = new List<Instance>();
        }

        internal Instance(object rootObject, Type instanceType, Instance parentInstance)
        {
            instanceIdentifier = new InstanceIdentifier((TypeIdentifier)instanceType.Identifier);
            this.parentInstance = parentInstance;
            childInstances = new List<Instance>();
        }
        #endregion

        #region Methods
        internal void AddChildInstance(Instance childInstance)
        {
            childInstances.Add(childInstance);
        }
        #endregion
    }
}
