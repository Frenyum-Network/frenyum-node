use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{distributions::Alphanumeric, thread_rng, rngs::ThreadRng};


fn random_message(rng: &mut ThreadRng) -> String
{
    let message: String = rng.sample_iter(&Alphanumeric)
        .take(256)
        .map(char::from)
        .collect::<String>();

    message
}

fn bench_signature_creation(c: &mut Criterion)
{
    c.bench_function("bench_signature_creation", |b| {
        b.iter(|| {
            let mut csprng: ThreadRng = thread_rng();
            let private_key = PrivateKey::generate(&mut csprng);
            let message = random_message(&mut csprng);
            let _signature = black_box(private_key.sign_message(message.as_bytes()));
        })
    });
}

fn bench_signature_verification(c: &mut Criterion)
{  
    let mut csprng: ThreadRng = thread_rng();
    let private_key = PrivateKey::generate(&mut csprng);
    let message = random_message(&mut csprng);
    let signature = private_key.sign_message(message.as_bytes());
    let public_key = private_key.to_public_key();

    c.bench_function("bench_signature_verification", |b| {
        b.iter(|| {
            let result = black_box(signature.verify(message.as_bytes(), &public_key));
            assert!(result.is_ok());
        })  
    });
}


criterion_group!(benches, bench_signature_creation, bench_signature_verification);
criterion_main!(benches);

