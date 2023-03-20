using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Identification;

    public interface ICSharpInstanceData : IInstanceData, ICSharpInstanceDataIdentifiable
    {
        #region Properties
        public ICSharpInstanceDataType CSharpInstanceDataType { get; }
        public ICSharpInstanceData? ParentCSharpInstanceData { get; }
        public SerializableList<ICSharpInstanceData> ChildCSharpInstanceData { get; }
        #endregion
    }
}
