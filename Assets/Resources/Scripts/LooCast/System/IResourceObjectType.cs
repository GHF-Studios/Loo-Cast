using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IResourceObjectType : IResourceType
    {
        #region Properties
        public IResourceObjectType ParentResourceObjectType { get; }
        public List<IResourceObjectType> ChildResourceObjectTypes { get; }
        #endregion
    }
}
