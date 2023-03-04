namespace LooCast.System
{
    using LooCast.System.Collections.Generic;

    public interface ICSharpInstanceMetaData : IInstanceMetaData, ICSharpInstanceMetaDataIdentifiable
    {
        #region Properties
        public ICSharpInstanceMetaData? ParentCSharpInstanceMetaData { get; }
        public SerializableList<ICSharpInstanceMetaData> ChildCSharpInstanceMetaData { get; }
        public ICSharpInstanceDataIdentifier CSharpInstanceDataIdentifier { get; set; }
        #endregion
    }
}
