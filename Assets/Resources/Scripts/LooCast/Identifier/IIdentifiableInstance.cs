namespace LooCast.Identifier
{
    public interface IIdentifiableInstance : IIdentifiableType
    {
        long InstanceID { get; }
    }
}