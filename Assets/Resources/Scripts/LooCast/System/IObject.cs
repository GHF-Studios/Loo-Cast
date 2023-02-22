using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identification;

    public interface IObject : IInstance, IObjectIdentifiable
    {
        #region Properties
        public IObjectType ObjectType { get; }
        public IObject ParentObject { get; }
        public List<IObject> ChildObjects { get; }
        #endregion
    }
}