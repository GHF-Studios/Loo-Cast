namespace LooCast.System
{
    public abstract class Factory<IdentifiableType, MetaDataType> where IdentifiableType : IIdentifiable where MetaDataType : IMetaData
    {
        public abstract IdentifiableType CreateInstance(MetaDataType metadata);
    }
}