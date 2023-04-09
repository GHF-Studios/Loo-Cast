namespace LooCast.System
{
    public abstract class Factory<IdentifiableType, MetaDataType> where IdentifiableType : IIdentifiable where MetaDataType : MetaData
    {
        public abstract IdentifiableType CreateInstance(MetaDataType metadata);
    }
}