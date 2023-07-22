namespace LooCast.System.Serialization
{
    public interface ISerializable
    {
        #region Methods
        IMetaData GetMetaData();
        void SetMetaData(IMetaData metaData);

        IData GetData();
        void SetData(IData data);
        #endregion
    }
}
