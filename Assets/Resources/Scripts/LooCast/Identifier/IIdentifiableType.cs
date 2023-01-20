namespace LooCast.Identifier
{
    public interface IIdentifiableType : IIdentifiable
    {
        string TypeName { get; }
    }   
}