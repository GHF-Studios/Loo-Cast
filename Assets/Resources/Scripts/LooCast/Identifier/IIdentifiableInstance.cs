using System;
using System.Collections.Generic;

namespace LooCast.Identifier
{
    public interface IIdentifiableInstance : IIdentifiable
    {
        #region Properties
        IIdentifiableInstance ParentInstance { get; }
        List<IIdentifiableInstance> ChildInstances { get; }
        IIdentifiableType InstanceType { get; }
        Guid InstanceID { get; }
        #endregion

        #region Methods
        void AddChildInstance(IIdentifiableInstance childInstance);
        void AddChildInstances(IEnumerable<IIdentifiableInstance> childInstances);
        void RemoveChildInstance(IIdentifiableInstance childInstance);
        #endregion
    }
}