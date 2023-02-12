using System;
using System.Collections.Generic;

namespace LooCast.Core.Instance
{
    using Identifier;

    public class Instance : IIdentifiable
    {
        #region Properties
        public IIdentifier Identifier => identifier;
        public Instance ParentInstance => parentInstance;
        public List<Instance> ChildInstances => childInstances;
        #endregion

        #region Fields
        private InstanceIdentifier identifier;
        private Instance parentInstance;
        private List<Instance> childInstances;
        #endregion

        #region Constructors
        internal Instance(TypeIdentifier instanceTypeIdentifier)
        {
            identifier = new InstanceIdentifier(instanceTypeIdentifier);
            parentInstance = null;
            childInstances = new List<Instance>();
        }

        internal Instance(TypeIdentifier instanceTypeIdentifier, Instance parentInstance)
        {
            identifier = new InstanceIdentifier(instanceTypeIdentifier);
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
