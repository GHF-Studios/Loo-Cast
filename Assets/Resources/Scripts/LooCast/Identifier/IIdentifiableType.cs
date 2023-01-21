using System.Collections.Generic;

namespace LooCast.Identifier
{
    public interface IIdentifiableType : IIdentifiable
    {
        #region Properties
        string TypeName { get; }
        IIdentifiableType ParentType { get; }
        List<IIdentifiableType> ChildTypes { get; }
        IIdentifiableNamespace TypeNamespace { get; }

        void AddChildType(IIdentifiableType childType);
        #endregion
    }
}