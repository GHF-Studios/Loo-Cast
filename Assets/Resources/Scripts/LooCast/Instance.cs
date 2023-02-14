using System;
using System.Collections.Generic;

namespace LooCast
{
    public class Instance : IIdentifiable
    {
        #region Properties
        public object RootObject => rootObject;
        public IIdentifier Identifier => identifier;
        public Instance ParentInstance => parentInstance;
        public List<Instance> ChildInstances => childInstances;
        #endregion

        #region Fields
        private object rootObject;
        private InstanceIdentifier identifier;
        private Instance parentInstance;
        private List<Instance> childInstances;
        #endregion

        #region Constructors
        internal Instance(object rootObject, Type instanceType)
        {
            identifier = new InstanceIdentifier((TypeIdentifier)instanceType.Identifier);
            parentInstance = null;
            childInstances = new List<Instance>();
        }

        internal Instance(object rootObject, Type instanceType, Instance parentInstance)
        {
            identifier = new InstanceIdentifier((TypeIdentifier)instanceType.Identifier);
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
