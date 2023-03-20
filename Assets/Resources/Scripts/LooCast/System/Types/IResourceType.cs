using System;
using System.Collections.Generic;

namespace LooCast.System.Types
{
    public interface IResourceType : IObjectType
    {
        #region Properties
        public IResourceType ParentResourceType { get; }
        public List<IResourceType> ChildResourceTypes { get; }
        #endregion
    }
}
