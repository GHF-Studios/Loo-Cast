using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identification;
    using LooCast.System.Types;

    public interface IObject : ICSharpInstance, IObjectIdentifiable
    {
        #region Properties
        public IObjectType ObjectType { get; }
        public IObject ParentObject { get; }
        public List<IObject> ChildObjects { get; }
        #endregion
    }
}