namespace LooCast.System.Serialization
{
    public interface ISerializable<MetaDataType, DataType> where MetaDataType : IMetaData where DataType : IData
    {
        #region Methods
        MetaDataType GetMetaData();
        void SetMetaData(MetaDataType metaData);

        DataType GetData();
        void SetData(DataType data);
        #endregion
    }
}
